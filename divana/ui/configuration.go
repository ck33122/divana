package ui

import (
	"log"

	"fyne.io/fyne/v2"
	"fyne.io/fyne/v2/container"
	"fyne.io/fyne/v2/layout"
	"fyne.io/fyne/v2/widget"
	"github.com/cprkv/divana/divana/config"
	"github.com/cprkv/divana/divana/sound"
)

type ConfigData struct {
	config         *config.Data
	inputDevices   []sound.DeviceInfo
	outputDevices  []sound.DeviceInfo
	inputDeviceId  int
	outputDeviceId int
}

var (
	configData           *ConfigData = nil
	inputDeviceConfigId              = "input-device"
	outputDeviceConfigId             = "output-device"
)

func findDeviceByIdStrOrDefault(id string, devs []sound.DeviceInfo) int {
	for i := range devs {
		if devs[i].IDStr == id {
			return i
		}
	}
	for i := range devs {
		if devs[i].IsDefault {
			return i
		}
	}
	panic("no suitable device found!")
}

func findDeviceByName(name string, devs []sound.DeviceInfo) int {
	for i := range devs {
		if devs[i].ToString(false) == name {
			return i
		}
	}
	for i := range devs {
		if devs[i].IsDefault {
			return i
		}
	}
	panic("no suitable device found!")
}

func currentInput() *sound.DeviceInfo {
	return &configData.inputDevices[configData.inputDeviceId]
}

func currentOutput() *sound.DeviceInfo {
	return &configData.outputDevices[configData.outputDeviceId]
}

func devicesStringArray(devs []sound.DeviceInfo) []string {
	result := []string{}
	for i := range devs {
		result = append(result, devs[i].ToString(false))
	}
	return result
}

func syncConfiguration() {
	if configData != nil {
		configData.config.InputDeviceId = currentInput().IDStr
		configData.config.OutputDeviceId = currentOutput().IDStr
		config.Save(configData.config)
	}

	configData = &ConfigData{
		config:         config.Load(),
		inputDevices:   sound.EnumerateCaptureDevices(),
		outputDevices:  sound.EnumeratePlaybackDevices(),
		inputDeviceId:  -1,
		outputDeviceId: -1,
	}

	log.Println()
	log.Println("input devices:")
	for _, dev := range configData.inputDevices {
		log.Printf("  %s", dev.ToString(false))
	}
	log.Println("output devices:")
	for _, dev := range configData.outputDevices {
		log.Printf("  %s", dev.ToString(false))
	}

	configData.inputDeviceId = findDeviceByIdStrOrDefault(configData.config.InputDeviceId, configData.inputDevices)
	configData.outputDeviceId = findDeviceByIdStrOrDefault(configData.config.OutputDeviceId, configData.outputDevices)
}

func InitConfiguration() {
	syncConfiguration()
}

func DeviceConfigDialogue() {
	window := application.NewWindow("Device configuration")
	window.Resize(fyne.NewSize(1024, 768))
	window.SetFixedSize(true)

	inputDeviceSelect := widget.NewSelect(devicesStringArray(configData.inputDevices), func(selection string) {
		configData.inputDeviceId = findDeviceByName(selection, configData.inputDevices)
	})
	inputDeviceSelect.SetSelected(currentInput().ToString(false))

	outputDeviceSelect := widget.NewSelect(devicesStringArray(configData.outputDevices), func(selection string) {
		configData.outputDeviceId = findDeviceByName(selection, configData.outputDevices)
	})
	outputDeviceSelect.SetSelected(currentOutput().ToString(false))

	content := container.NewVBox(
		widget.NewForm(
			widget.NewFormItem("Input device", inputDeviceSelect),
			widget.NewFormItem("Output device", outputDeviceSelect),
		),
		container.NewHBox(
			layout.NewSpacer(),
			widget.NewButton("cancel", func() {
				configData = nil
				syncConfiguration()
				window.Close()
			}),
			widget.NewButton("apply", func() {
				syncConfiguration()
				window.Close()
			}),
		),
	)

	window.SetContent(content)
	window.ShowAndRun()
}
