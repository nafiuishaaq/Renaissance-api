import { EventsHandler, IEventHandler } from '@nestjs/cqrs';
import { BetPlacedEvent } from '../domain/events';
import { LeaderboardService } from '../leaderboard.service';

/**
 * Event handler for BetPlacedEvent
 * Updates leaderboard activity tracking when a bet is placed
 */
@EventsHandler(BetPlacedEvent)
export class BetPlacedEventHandler implements IEventHandler<BetPlacedEvent> {
  constructor(private readonly leaderboardService: LeaderboardService) {}

  async handle(event: BetPlacedEvent): Promise<void> {
    await this.leaderboardService.handleBetPlaced(event);
  }
}
