namespace divana.client.ui {
  public abstract class UiActionResult {
    public sealed class Exit : UiActionResult { }

    public sealed class SwitchLayout : UiActionResult {
      public UiLayout layout { get; }

      public SwitchLayout(UiLayout layout) {
        this.layout = layout;
      }
    }

    public sealed class None : UiActionResult { }
  }
}
