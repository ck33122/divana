package config

import (
	"gopkg.in/ini.v1"
)

type Data struct {
	OutputDeviceId string
	InputDeviceId  string
}

const (
	configName = "divana-config.ini"
)

func Save(data *Data) {
	cfg := ini.Empty()
	cfg.Section("").Key("OutputDeviceId").SetValue(data.OutputDeviceId)
	cfg.Section("").Key("InputDeviceId").SetValue(data.InputDeviceId)
	err := cfg.SaveTo(configName)
	if err != nil {
		panic(err)
	}
}

func Load() *Data {
	cfg, err := ini.Load(configName)
	if err != nil {
		return &Data{
			OutputDeviceId: "",
			InputDeviceId:  "",
		}
	}
	return &Data{
		OutputDeviceId: cfg.Section("").Key("OutputDeviceId").String(),
		InputDeviceId:  cfg.Section("").Key("InputDeviceId").String(),
	}
}
