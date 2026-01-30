import { EventsHandler, IEventHandler } from '@nestjs/cqrs';
import { BetSettledEvent } from '../domain/events';
import { LeaderboardService } from '../leaderboard.service';

/**
 * Event handler for BetSettledEvent
 * Updates leaderboard stats including accuracy and winning streak
 * This is where accuracy is recalculated (only on settlement, not placement)
 */
@EventsHandler(BetSettledEvent)
export class BetSettledEventHandler implements IEventHandler<BetSettledEvent> {
  constructor(private readonly leaderboardService: LeaderboardService) {}

  async handle(event: BetSettledEvent): Promise<void> {
    await this.leaderboardService.handleBetSettled(event);
  }
}
