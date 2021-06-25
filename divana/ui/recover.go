package ui

import (
	"fmt"

	"github.com/gonutz/wui/v2"
)

func HandleMainRecover() {
	if r := recover(); r != nil {
		wui.MessageBoxError("Fatal Error", fmt.Sprintf("%v", r))
		panic(r)
	}
}
