package main

import (
	"github.com/cprkv/divana/divana/config"
	"github.com/cprkv/divana/divana/sound"
	"github.com/cprkv/divana/divana/ui"
)

var (
	cfg *config.Data
)

func main() {
	defer ui.HandleMainRecover("Divana")

	cfg = config.Load()

	sound.ContextInit()
	defer sound.ContextDestroy()

	ui.InitConfiguration()
	ui.DeviceConfigDialogue()
}
