export class LoadingCount {
  private count = 0;
  public addCount() {
    this.count++;
    this.showLoad();
    return this.count;
  }
  public decreaseCount() {
    this.count--;
    this.showLoad();
    return this.count;
  }
  private showLoad() {
    // const [messageApi, contextHolder] = message.useMessage();
    if (this.count > 0) {
      // 应该放loading的位置
    } else {
      // eventBus.emit('setLoadingStatus', false)
    }
  }
  public getCount(): number {
    return this.count;
  }
}
