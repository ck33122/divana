package ui

import (
	"fyne.io/fyne/v2"
	"fyne.io/fyne/v2/app"
	"fyne.io/fyne/v2/container"
	"fyne.io/fyne/v2/widget"
)

var (
	application fyne.App
)

func Example() {
	window := application.NewWindow("Hello")
	hello := widget.NewLabel("Hello Fyne!")
	window.SetContent(container.NewVBox(
		hello,
		widget.NewButton("Hi!", func() {
			hello.SetText("Welcome :)")
		}),
	))
	window.ShowAndRun()
}

func init() {
	application = app.New()
}
