package main

import (
	"github.com/cprkv/divana/divana/config"
	"github.com/cprkv/divana/divana/ui"
)

var (
	cfg *config.Data
)

func main() {
	defer ui.HandleMainRecover()
	cfg = config.Load()

	window := ui.CreateWindow("Divana", 640, 480)
	
	mainDiskSelectPanel := window.AddPanelLineNext()
	mainDiskSelectEditor := window.AddFolderSelectChild(mainDiskSelectPanel, "Main recovery disk path", cfg.MainDiskPath)
	mainDiskSelectEditor.SetOnTextChange(func() {
		cfg.MainDiskPath = mainDiskSelectEditor.Text()
		config.Save(cfg)
	})

	driverDiskSelectPanel := window.AddPanelLineNext()
	driverDiskSelectEditor := window.AddFolderSelectChild(driverDiskSelectPanel, "Driver disk path", cfg.DriverDiskPath)
	driverDiskSelectEditor.SetOnTextChange(func() {
		cfg.DriverDiskPath = driverDiskSelectEditor.Text()
		config.Save(cfg)
	})
	
	window.Show()
}


