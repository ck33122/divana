package sound

import (
	"log"

	"github.com/gen2brain/malgo"
)

var (
	context *malgo.AllocatedContext
)

func ContextInit() {
	var err error
	context, err = malgo.InitContext(
		[]malgo.Backend{malgo.BackendCoreaudio, malgo.BackendWasapi},
		malgo.ContextConfig{},
		func(message string) { log.Printf("[malgo] %s", message) },
	)
	if err != nil {
		panic(err)
	}
}

func ContextDestroy() {
	context.Uninit()
	context.Free()
}
