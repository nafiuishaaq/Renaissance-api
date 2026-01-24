import { Module } from '@nestjs/common';
import { CqrsModule } from '@nestjs/cqrs';
import { TypeOrmModule } from '@nestjs/typeorm';
import { LeaderboardService } from './leaderboard.service';
import { LeaderboardController } from './leaderboard.controller';
import { Leaderboard } from './entities/leaderboard.entity';
import { User } from '../users/entities/user.entity';
import {
  BetPlacedEventHandler,
  BetSettledEventHandler,
  StakeCreditedEventHandler,
  StakeDebitedEventHandler,
} from './listeners';

const EVENT_HANDLERS = [
  BetPlacedEventHandler,
  BetSettledEventHandler,
  StakeCreditedEventHandler,
  StakeDebitedEventHandler,
];

@Module({
  imports: [CqrsModule, TypeOrmModule.forFeature([Leaderboard, User])],
  controllers: [LeaderboardController],
  providers: [LeaderboardService, ...EVENT_HANDLERS],
  exports: [LeaderboardService],
})
export class LeaderboardModule {}
