/**
 * Domain event emitted when staked tokens are debited (e.g., unstaking, penalties)
 * Triggers leaderboard stats update
 */
export class StakeDebitedEvent {
  constructor(
    public readonly userId: string,
    public readonly stakedAmount: number,
    public readonly reason: string, // e.g., 'unstake', 'penalty'
    public readonly timestamp: Date = new Date(),
  ) {}
}
