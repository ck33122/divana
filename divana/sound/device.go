package sound

import (
	"fmt"
	"log"

	"github.com/gen2brain/malgo"
)

type DeviceInfo struct {
	ID            malgo.DeviceID
	IDStr         string
	Type          malgo.DeviceType
	Name          string
	IsDefault     bool
	Formats       []uint32
	MinChannels   uint32
	MaxChannels   uint32
	MinSampleRate uint32
	MaxSampleRate uint32
}

func (dev *DeviceInfo) ToString() string {
	channelInfo := fmt.Sprintf("%vch-%vch", dev.MinChannels, dev.MaxChannels)
	if dev.MinChannels == dev.MaxChannels {
		channelInfo = fmt.Sprintf("%vch", dev.MinChannels)
	}
	sampleRate := fmt.Sprintf("%vhz-%vhz", dev.MinSampleRate, dev.MaxSampleRate)
	if dev.MinSampleRate == dev.MaxSampleRate {
		sampleRate = fmt.Sprintf("%vhz", dev.MinSampleRate)
	}
	return fmt.Sprintf("[%v] %s [%v] [%s], [%s]", dev.IDStr, dev.Name, dev.IsDefault, channelInfo, sampleRate)
}

func EnumeratePlaybackDevices() []DeviceInfo {
	infos, err := context.Devices(malgo.Playback)
	if err != nil {
		panic(err)
	}
	result := []DeviceInfo{}
	for _, info := range infos {
		full, err := context.DeviceInfo(malgo.Playback, info.ID, malgo.Shared)
		if err != nil {
			log.Printf("error get playback device info: %v\n", err)
			continue
		}
		deviceInfo := DeviceInfo{
			ID:            info.ID,
			IDStr:         idToString(info.ID),
			Type:          malgo.Playback,
			Name:          info.Name()[0:strlen(info.Name())],
			IsDefault:     full.IsDefault != 0,
			Formats:       full.Formats[0:full.FormatCount],
			MinChannels:   full.MinChannels,
			MaxChannels:   full.MaxChannels,
			MinSampleRate: full.MinSampleRate,
			MaxSampleRate: full.MaxSampleRate,
		}
		result = append(result, deviceInfo)
	}
	return result
}

func EnumerateCaptureDevices() []DeviceInfo {
	infos, err := context.Devices(malgo.Capture)
	if err != nil {
		panic(err)
	}
	result := []DeviceInfo{}
	for _, info := range infos {
		full, err := context.DeviceInfo(malgo.Capture, info.ID, malgo.Shared)
		if err != nil {
			log.Printf("error capture device info: %v\n", err)
			continue
		}
		deviceInfo := DeviceInfo{
			ID:            info.ID,
			IDStr:         idToString(info.ID),
			Type:          malgo.Capture,
			Name:          info.Name()[0:strlen(info.Name())],
			IsDefault:     full.IsDefault != 0,
			Formats:       full.Formats[0:full.FormatCount],
			MinChannels:   full.MinChannels,
			MaxChannels:   full.MaxChannels,
			MinSampleRate: full.MinSampleRate,
			MaxSampleRate: full.MaxSampleRate,
		}
		result = append(result, deviceInfo)
	}
	return result
}

func strlen(s string) int {
	for i := 0; ; i++ {
		if s[i] == 0 {
			return i
		}
	}
}

func idToString(id malgo.DeviceID) string {
	withoutZeroes := []byte{}
	for i := range id {
		if id[i] != 0 {
			withoutZeroes = append(withoutZeroes, id[i])
		}
	}
	withoutZeroes = append(withoutZeroes, 0)
	len := strlen(string(withoutZeroes[:]))
	str := string(withoutZeroes[0:len])
	return str
}
