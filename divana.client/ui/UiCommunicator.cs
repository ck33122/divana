using System;

namespace divana.client.ui {
  public sealed class UiCommunicator {
    private readonly string layoutName;

    public UiCommunicator(string layoutName) {
      this.layoutName = layoutName;
    }

    public UiCommunicator createChild(string childLayoutName) =>
      new UiCommunicator($"{layoutName}>{childLayoutName}");

    public void showMessage(string message) =>
      Console.WriteLine($"{layoutName}> {message}");

    public string readInput() {
      Console.Write($"{layoutName}> ");
      return Console.ReadLine()?.Trim() ?? "";
    }
  }
}
