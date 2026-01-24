import { Controller, Get, Param, Query } from '@nestjs/common';
import { LeaderboardService } from './leaderboard.service';
import { Leaderboard } from './entities/leaderboard.entity';

@Controller('leaderboards')
export class LeaderboardController {
  constructor(private readonly leaderboardService: LeaderboardService) {}

  /**
   * Get user's leaderboard stats
   */
  @Get('users/:userId')
  async getUserLeaderboard(
    @Param('userId') userId: string,
  ): Promise<Leaderboard | null> {
    return this.leaderboardService.getLeaderboardStats(userId);
  }

  /**
   * Get top leaderboard entries
   * Query params:
   * - limit: number of results (default: 100)
   * - orderBy: 'totalWinnings' | 'bettingAccuracy' | 'winningStreak' (default: 'totalWinnings')
   */
  @Get('top')
  async getTopLeaderboard(
    @Query('limit') limit: string = '100',
    @Query('orderBy')
    orderBy: 'totalWinnings' | 'bettingAccuracy' | 'winningStreak' =
      'totalWinnings',
  ): Promise<Leaderboard[]> {
    return this.leaderboardService.getTopLeaderboard(
      parseInt(limit, 10),
      orderBy,
    );
  }
}
