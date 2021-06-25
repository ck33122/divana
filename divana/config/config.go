package config

import (
	"gopkg.in/ini.v1"
)

type Data struct {
	MainDiskPath   string
	DriverDiskPath string
}

const (
	configName = "divana-config.ini"
)

func Save(data *Data) {
	cfg := ini.Empty()
	cfg.Section("").Key("MainDiskPath").SetValue(data.MainDiskPath)
	cfg.Section("").Key("DriverDiskPath").SetValue(data.DriverDiskPath)
	err := cfg.SaveTo(configName)
	if err != nil {
		panic(err)
	}
}

func Load() *Data {
	cfg, err := ini.Load(configName)
	if err != nil {
		return &Data{
			MainDiskPath:   "",
			DriverDiskPath: "",
		}
	}
	return &Data{
		MainDiskPath:   cfg.Section("").Key("MainDiskPath").String(),
		DriverDiskPath: cfg.Section("").Key("DriverDiskPath").String(),
	}
}
