/**
 * Domain event emitted when stake rewards are credited to a user
 * Triggers leaderboard stats update
 */
export class StakeCreditedEvent {
  constructor(
    public readonly userId: string,
    public readonly stakedAmount: number,
    public readonly rewardAmount: number,
    public readonly timestamp: Date = new Date(),
  ) {}
}
