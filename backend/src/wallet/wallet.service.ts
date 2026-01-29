import {
  Injectable,
  BadRequestException,
  NotFoundException,
  ConflictException,
  InternalServerErrorException,
} from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository, DataSource, QueryRunner } from 'typeorm';
import { Balance } from './entities/balance.entity';
import {
  BalanceTransaction,
  TransactionType,
  TransactionSource,
} from './entities/balance-transaction.entity';
import { User } from '../users/entities/user.entity';

@Injectable()
export class WalletService {
  constructor(
    @InjectRepository(Balance)
    private readonly balanceRepository: Repository<Balance>,
    @InjectRepository(BalanceTransaction)
    private readonly transactionRepository: Repository<BalanceTransaction>,
    private readonly dataSource: DataSource,
  ) {}

  async createWallet(userId: string): Promise<Balance> {
    const existingBalance = await this.balanceRepository.findOne({
      where: { userId },
    });
    if (existingBalance) {
      return existingBalance;
    }

    const balance = this.balanceRepository.create({
      userId,
      availableBalance: 0,
      lockedBalance: 0,
    });
    return await this.balanceRepository.save(balance);
  }

  async getBalance(userId: string): Promise<Balance> {
    const balance = await this.balanceRepository.findOne({ where: { userId } });
    if (!balance) {
      return this.createWallet(userId);
    }
    return balance;
  }

  async credit(
    userId: string,
    amount: number,
    source: TransactionSource,
    referenceId: string,
    metadata?: Record<string, any>,
  ): Promise<Balance> {
    if (amount <= 0) throw new BadRequestException('Amount must be positive');

    return this.performAtomicOperation(userId, async (queryRunner, balance) => {
      if (referenceId) {
        const existing = await queryRunner.manager.findOne(BalanceTransaction, {
          where: { referenceId, type: TransactionType.CREDIT, source },
        });
        if (existing)
          throw new ConflictException('Transaction already processed');
      }

      const previousBalance = balance.availableBalance;
      balance.availableBalance += amount;

      const transaction = queryRunner.manager.create(BalanceTransaction, {
        balanceId: balance.id,
        amount,
        type: TransactionType.CREDIT,
        source,
        referenceId,
        metadata,
        previousBalance,
        newBalance: balance.availableBalance,
      });

      await queryRunner.manager.save(transaction);
      return await queryRunner.manager.save(balance);
    });
  }

  async debit(
    userId: string,
    amount: number,
    source: TransactionSource,
    referenceId: string,
    metadata?: Record<string, any>,
  ): Promise<Balance> {
    if (amount <= 0) throw new BadRequestException('Amount must be positive');

    return this.performAtomicOperation(userId, async (queryRunner, balance) => {
      if (referenceId) {
        const existing = await queryRunner.manager.findOne(BalanceTransaction, {
          where: { referenceId, type: TransactionType.DEBIT, source },
        });
        if (existing)
          throw new ConflictException('Transaction already processed');
      }

      if (balance.availableBalance < amount) {
        throw new ConflictException('Insufficient funds');
      }

      const previousBalance = balance.availableBalance;
      balance.availableBalance -= amount;

      const transaction = queryRunner.manager.create(BalanceTransaction, {
        balanceId: balance.id,
        amount,
        type: TransactionType.DEBIT,
        source,
        referenceId,
        metadata,
        previousBalance,
        newBalance: balance.availableBalance,
      });

      await queryRunner.manager.save(transaction);
      return await queryRunner.manager.save(balance);
    });
  }

  async lockFunds(
    userId: string,
    amount: number,
    referenceId: string,
  ): Promise<Balance> {
    if (amount <= 0) throw new BadRequestException('Amount must be positive');

    return this.performAtomicOperation(userId, async (queryRunner, balance) => {
      if (balance.availableBalance < amount) {
        throw new ConflictException('Insufficient available funds to lock');
      }

      balance.availableBalance -= amount;
      balance.lockedBalance += amount;

      const transaction = queryRunner.manager.create(BalanceTransaction, {
        balanceId: balance.id,
        amount,
        type: TransactionType.DEBIT,
        source: TransactionSource.BET,
        referenceId,
        metadata: { action: 'LOCK_FUNDS' },
        previousBalance: balance.availableBalance + amount,
        newBalance: balance.availableBalance,
      });

      await queryRunner.manager.save(transaction);
      return await queryRunner.manager.save(balance);
    });
  }

  async unlockFunds(
    userId: string,
    amount: number,
    referenceId: string,
  ): Promise<Balance> {
    if (amount <= 0) throw new BadRequestException('Amount must be positive');

    return this.performAtomicOperation(userId, async (queryRunner, balance) => {
      if (balance.lockedBalance < amount) {
        throw new ConflictException('Insufficient locked funds');
      }

      balance.lockedBalance -= amount;
      balance.availableBalance += amount;

      const transaction = queryRunner.manager.create(BalanceTransaction, {
        balanceId: balance.id,
        amount,
        type: TransactionType.CREDIT,
        source: TransactionSource.BET,
        referenceId,
        metadata: { action: 'UNLOCK_FUNDS' },
        previousBalance: balance.availableBalance - amount,
        newBalance: balance.availableBalance,
      });

      await queryRunner.manager.save(transaction);
      return await queryRunner.manager.save(balance);
    });
  }

  async consumeLockedFunds(
    userId: string,
    amount: number,
    referenceId: string,
  ): Promise<Balance> {
    if (amount <= 0) throw new BadRequestException('Amount must be positive');

    return this.performAtomicOperation(userId, async (queryRunner, balance) => {
      if (balance.lockedBalance < amount) {
        throw new ConflictException('Insufficient locked funds to consume');
      }

      balance.lockedBalance -= amount;

      const transaction = queryRunner.manager.create(BalanceTransaction, {
        balanceId: balance.id,
        amount,
        type: TransactionType.DEBIT,
        source: TransactionSource.BET,
        referenceId,
        metadata: { action: 'CONSUME_LOCKED' },
        previousBalance: balance.lockedBalance + amount,
        newBalance: balance.lockedBalance,
      });

      await queryRunner.manager.save(transaction);
      return await queryRunner.manager.save(balance);
    });
  }

  async getTransactionHistory(
    userId: string,
    page: number = 1,
    limit: number = 10,
  ): Promise<{ data: BalanceTransaction[]; total: number }> {
    const balance = await this.getBalance(userId);

    const [data, total] = await this.transactionRepository.findAndCount({
      where: { balanceId: balance.id },
      order: { createdAt: 'DESC' },
      skip: (page - 1) * limit,
      take: limit,
    });

    return { data, total };
  }

  private async performAtomicOperation(
    userId: string,
    operation: (queryRunner: QueryRunner, balance: Balance) => Promise<Balance>,
  ): Promise<Balance> {
    const queryRunner = this.dataSource.createQueryRunner();
    await queryRunner.connect();
    await queryRunner.startTransaction();

    try {
      let balance = await queryRunner.manager.findOne(Balance, {
        where: { userId },
        lock: { mode: 'pessimistic_write' },
      });

      if (!balance) {
        const newBalance = queryRunner.manager.create(Balance, {
          userId,
          availableBalance: 0,
          lockedBalance: 0,
        });
        balance = await queryRunner.manager.save(newBalance);
      }

      const result = await operation(queryRunner, balance);

      await queryRunner.commitTransaction();
      return result;
    } catch (err) {
      await queryRunner.rollbackTransaction();
      throw err;
    } finally {
      await queryRunner.release();
    }
  }
}
