using System.Linq;

namespace divana.client.ui {
  public sealed class UiLayout {
    private readonly string name;
    private readonly UiAction[] actions;

    public UiLayout(string name, params UiAction[] actions) {
      this.name = name;
      this.actions = actions;
    }

    public void run(UiCommunicator communicator = null) {
      communicator ??= new UiCommunicator(name);
      void showHelp() => communicator.showMessage($"available commands: {string.Join(',', actions.Select(x => x.name))}");

      showHelp();

      while (true) {
        var actionName = communicator.readInput().ToLowerInvariant();
        if (actionName == "?" || actionName == "help" || actionName == "??") {
          showHelp();
          continue;
        }

        var action = actions.FirstOrDefault(x => x.name == actionName);
        if (action == null) {
          communicator.showMessage($"command {actionName} is unknown!");
          showHelp();
          continue;
        }

        var result = action.run(communicator);
        switch (result) {
          case UiActionResult.Exit _:
            return;
          case UiActionResult.SwitchLayout switchLayout:
            switchLayout.layout.run(communicator.createChild(switchLayout.layout.name));
            break;
        }
      }
    }
  }
}
