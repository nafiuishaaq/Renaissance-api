/**
 * Domain event emitted when a bet is settled (won or lost)
 * Triggers leaderboard stats update including accuracy and winning streak
 */
export class BetSettledEvent {
  constructor(
    public readonly userId: string,
    public readonly betId: string,
    public readonly matchId: string,
    public readonly isWin: boolean,
    public readonly stakeAmount: number,
    public readonly winningsAmount: number, // 0 if loss
    public readonly accuracy: number, // Betting accuracy percentage
    public readonly timestamp: Date = new Date(),
  ) {}
}
