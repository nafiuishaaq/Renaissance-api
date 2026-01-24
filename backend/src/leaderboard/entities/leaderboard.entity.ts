import {
  Entity,
  PrimaryGeneratedColumn,
  Column,
  CreateDateColumn,
  UpdateDateColumn,
  OneToOne,
  JoinColumn,
  Index,
} from 'typeorm';
import { User } from '../../users/entities/user.entity';

/**
 * Leaderboard entity to track user betting and staking stats
 * All updates are atomic to ensure data consistency
 */
@Entity('leaderboards')
@Index(['userId'], { unique: true })
@Index(['totalWinnings'])
@Index(['bettingAccuracy'])
@Index(['winningStreak'])
export class Leaderboard {
  @PrimaryGeneratedColumn('uuid')
  id: string;

  @OneToOne(() => User, { onDelete: 'CASCADE' })
  @JoinColumn({ name: 'userId' })
  user: User;

  @Column('uuid')
  userId: string;

  // Betting Statistics
  @Column('int', { default: 0 })
  totalBets: number; // Total number of bets placed

  @Column('int', { default: 0 })
  betsWon: number; // Number of winning bets

  @Column('int', { default: 0 })
  betsLost: number; // Number of losing bets

  @Column('decimal', { precision: 10, scale: 2, default: 0 })
  totalWinnings: number; // Total winnings from bets

  @Column('decimal', { precision: 5, scale: 2, default: 0 })
  bettingAccuracy: number; // Percentage of winning bets (0-100)

  @Column('int', { default: 0 })
  winningStreak: number; // Current winning streak count

  @Column('int', { default: 0 })
  highestWinningStreak: number; // Highest winning streak achieved

  // Staking Statistics
  @Column('decimal', { precision: 10, scale: 2, default: 0 })
  totalStaked: number; // Total amount staked

  @Column('decimal', { precision: 10, scale: 2, default: 0 })
  totalStakingRewards: number; // Total rewards from staking

  @Column('decimal', { precision: 10, scale: 2, default: 0 })
  activeStakes: number; // Currently active staked amount

  // Activity Tracking
  @Column('timestamp', { nullable: true })
  lastBetAt: Date; // Timestamp of last bet placed

  @Column('timestamp', { nullable: true })
  lastStakeAt: Date; // Timestamp of last stake transaction

  @CreateDateColumn()
  createdAt: Date;

  @UpdateDateColumn()
  updatedAt: Date;

  /**
   * Recalculate betting accuracy based on wins and total bets
   * Called after each bet settlement
   */
  recalculateAccuracy(): void {
    if (this.totalBets === 0) {
      this.bettingAccuracy = 0;
    } else {
      this.bettingAccuracy = Number(
        ((this.betsWon / this.totalBets) * 100).toFixed(2),
      );
    }
  }

  /**
   * Update winning streak based on bet result
   */
  updateWinningStreak(isWin: boolean): void {
    if (isWin) {
      this.winningStreak++;
      if (this.winningStreak > this.highestWinningStreak) {
        this.highestWinningStreak = this.winningStreak;
      }
    } else {
      this.winningStreak = 0;
    }
  }
}
