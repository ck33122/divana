namespace divana.client.ui {
  public abstract class UiAction {
    public abstract string name { get; }
    public abstract UiActionResult run(UiCommunicator communicator);

    public sealed class Exit : UiAction {
      public override string name => "exit";
      public override UiActionResult run(UiCommunicator communicator) => new UiActionResult.Exit();
    }
  }
}
