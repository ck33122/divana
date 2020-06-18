using System;
using divana.client.ui;

namespace divana.client {
  internal static class Program {
    // ReSharper disable once InconsistentNaming
    public static void Main(string[] args) {
      Console.WriteLine("divana 1.0");
      UiLayouts.login.run();
      Console.WriteLine("exiting!");
    }
  }
}
