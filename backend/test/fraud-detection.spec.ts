describe('Fraud Detection Service', () => {
  it('calculates risk score correctly', async () => {
    const service = new FraudDetectionService();
    const score = await service.calculateRiskScore('device123', '192.168.0.1', { ua: 'test' });
    expect(typeof score).toBe('number');
  });
});
