/**
 * Domain event emitted when a bet is placed
 * Triggers leaderboard activity tracking
 */
export class BetPlacedEvent {
  constructor(
    public readonly userId: string,
    public readonly matchId: string,
    public readonly stakeAmount: number,
    public readonly predictedOutcome: string,
    public readonly timestamp: Date = new Date(),
  ) {}
}
