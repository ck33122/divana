package ui

import (
	"log"

	"github.com/gonutz/wui/v2"
)

type Window struct {
	window       *wui.Window
	screenW      int
	screenH      int
	buttonW      int
	buttonH      int
	commonMargin int
}

func CreateWindow(title string, width, height int) *Window {
	window := wui.NewWindow()
	window.SetResizable(false)
	window.SetHasMaxButton(false)
	window.SetTitle(title)
	window.SetSize(width, height)
	return &Window{
		window:       window,
		screenW:      width,
		screenH:      height,
		buttonW:      80,
		buttonH:      25,
		commonMargin: 15,
	}
}

func (wnd *Window) Show() {
	wnd.window.Show()
}

func (wnd *Window) GetLastChildren() wui.Control {
	children := wnd.window.Children()
	if len(children) == 0 {
		return nil
	}
	return children[len(children)-1]
}

func (wnd *Window) AddPanelLineNext() *wui.Panel {
	lastChildren := wnd.GetLastChildren()
	panel := wui.NewPanel()
	panX, panY := wnd.commonMargin, wnd.commonMargin
	if lastChildren != nil {
		_, by, _, bh := lastChildren.Bounds()
		panY = by + bh + wnd.commonMargin
	}
	panel.SetPosition(panX, panY)
	panel.SetSize(wnd.window.InnerWidth(), wnd.buttonH)
	wnd.window.Add(panel)
	return panel
}

func (wnd *Window) AddFolderSelectChild(panel *wui.Panel, labelText, defaultPath string) *wui.EditLine {
	var rightXButton = panel.InnerWidth() - wnd.buttonW - wnd.commonMargin*2
	var labelW = 180

	label := wui.NewLabel()
	label.SetPosition(0, 0)
	label.SetSize(labelW, wnd.buttonH)
	label.SetText(labelText)
	panel.Add(label)

	editLine := wui.NewEditLine()
	editLine.SetPosition(labelW, 0)
	editLine.SetSize(rightXButton-wnd.commonMargin-labelW, wnd.buttonH)
	editLine.SetText(defaultPath)
	panel.Add(editLine)

	button := wui.NewButton()
	button.SetPosition(rightXButton, 0)
	button.SetSize(wnd.buttonW, wnd.buttonH)
	button.SetText("Browse")
	button.SetOnClick(func() {
		dialog := wui.NewFolderSelectDialog()
		dialog.SetTitle("Select directory for " + labelText)
		ok, newPath := dialog.Execute(wnd.window)
		if ok {
			editLine.SetText(newPath)
			log.Printf("newPath: %s\n", newPath)
		}
	})
	panel.Add(button)

	return editLine
}
