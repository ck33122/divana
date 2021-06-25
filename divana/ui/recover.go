package ui

import (
	"fmt"

	"fyne.io/fyne/v2"
	"fyne.io/fyne/v2/container"
	"fyne.io/fyne/v2/widget"
)

func HandleMainRecover(appname string) {
	if r := recover(); r != nil {
		window := application.NewWindow(appname + " Fatal Error")
		window.SetContent(
			container.NewVBox(
				widget.NewLabelWithStyle("Fatal error happend. Execution stopped!", fyne.TextAlignLeading, fyne.TextStyle{Bold: true}),
				container.NewPadded(widget.NewTextGridFromString(fmt.Sprintf("%v", r))),
				container.NewGridWithColumns(
					2,
					widget.NewLabel(""),
					widget.NewButton("Exit application", func() { window.Close() }),
				),
			),
		)
		window.Resize(fyne.NewSize(800, 0))
		window.ShowAndRun()
		panic(r)
	}
}
