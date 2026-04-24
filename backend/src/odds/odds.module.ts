import { Module } from '@nestjs/common';
import { TypeOrmModule } from '@nestjs/typeorm';
import { CacheModule } from '@nestjs/cache-manager';
import { HttpModule } from '@nestjs/axios';
import { Bet } from '../bets/entities/bet.entity';
import { Match } from '../matches/entities/match.entity';
import { OddsController } from './odds.controller';
import { MatchOddsHistory } from './entities/match-odds-history.entity';
import { OddsRealtimeService } from './odds-realtime.service';
import { OddsService } from './odds.service';

@Module({
  imports: [
    TypeOrmModule.forFeature([Match, Bet, MatchOddsHistory]),
    CacheModule.register({}),
    HttpModule,
  ],
  controllers: [OddsController],
  providers: [OddsService, OddsRealtimeService],
  exports: [OddsService],
})
export class OddsModule {}
