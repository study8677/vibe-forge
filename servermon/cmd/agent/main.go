package main

import (
	"bytes"
	"encoding/json"
	"flag"
	"fmt"
	"log"
	"net/http"
	"os"
	"runtime"
	"strings"
	"time"

	"github.com/shirou/gopsutil/v3/cpu"
	"github.com/shirou/gopsutil/v3/disk"
	"github.com/shirou/gopsutil/v3/host"
	"github.com/shirou/gopsutil/v3/load"
	"github.com/shirou/gopsutil/v3/mem"
	"github.com/shirou/gopsutil/v3/net"
	"github.com/shirou/gopsutil/v3/process"

	"servermon/internal/model"
)

func main() {
	serverURL := flag.String("s", "http://localhost:8080", "server URL")
	secretKey := flag.String("k", "", "server secret key")
	interval := flag.Int("i", 5, "report interval in seconds")
	flag.Parse()

	if *secretKey == "" {
		fmt.Fprintln(os.Stderr, "Error: secret key is required (-k)")
		flag.Usage()
		os.Exit(1)
	}

	reportURL := strings.TrimRight(*serverURL, "/") + "/api/agent/report"
	log.Printf("Agent starting: server=%s interval=%ds", *serverURL, *interval)

	hostInfo := collectHostInfo()
	var prevNetIn, prevNetOut uint64
	firstRun := true

	ticker := time.NewTicker(time.Duration(*interval) * time.Second)
	defer ticker.Stop()

	// first report immediately
	report(reportURL, *secretKey, hostInfo, collectMetrics(&prevNetIn, &prevNetOut, *interval, firstRun))
	firstRun = false

	for range ticker.C {
		metrics := collectMetrics(&prevNetIn, &prevNetOut, *interval, firstRun)
		report(reportURL, *secretKey, hostInfo, metrics)
	}
}

func collectHostInfo() model.HostInfo {
	info := model.HostInfo{
		Arch: runtime.GOARCH,
	}

	if hi, err := host.Info(); err == nil {
		info.Platform = hi.Platform + " " + hi.PlatformVersion
		info.Version = hi.KernelVersion
	}

	cpuInfos, err := cpu.Info()
	if err == nil && len(cpuInfos) > 0 {
		info.CPUInfo = cpuInfos[0].ModelName
	}

	return info
}

func collectMetrics(prevNetIn, prevNetOut *uint64, interval int, firstRun bool) model.Metrics {
	m := model.Metrics{}

	// CPU
	if percents, err := cpu.Percent(time.Second, false); err == nil && len(percents) > 0 {
		m.CPU = percents[0]
	}

	// Memory
	if vm, err := mem.VirtualMemory(); err == nil {
		m.MemTotal = vm.Total
		m.MemUsed = vm.Used
	}

	// Swap
	if sm, err := mem.SwapMemory(); err == nil {
		m.SwapTotal = sm.Total
		m.SwapUsed = sm.Used
	}

	// Disk (root partition)
	if du, err := disk.Usage("/"); err == nil {
		m.DiskTotal = du.Total
		m.DiskUsed = du.Used
	}

	// Network
	if counters, err := net.IOCounters(false); err == nil && len(counters) > 0 {
		currentIn := counters[0].BytesRecv
		currentOut := counters[0].BytesSent
		m.NetInTotal = currentIn
		m.NetOutTotal = currentOut

		if !firstRun && interval > 0 {
			m.NetInSpeed = (currentIn - *prevNetIn) / uint64(interval)
			m.NetOutSpeed = (currentOut - *prevNetOut) / uint64(interval)
		}
		*prevNetIn = currentIn
		*prevNetOut = currentOut
	}

	// Load
	if avg, err := load.Avg(); err == nil {
		m.Load1 = avg.Load1
		m.Load5 = avg.Load5
		m.Load15 = avg.Load15
	}

	// Process count
	if pids, err := process.Pids(); err == nil {
		m.ProcessCount = uint64(len(pids))
	}

	// Uptime
	if up, err := host.Uptime(); err == nil {
		m.Uptime = up
	}

	// TCP/UDP counts
	if conns, err := net.Connections("tcp"); err == nil {
		m.TCPCount = uint32(len(conns))
	}
	if conns, err := net.Connections("udp"); err == nil {
		m.UDPCount = uint32(len(conns))
	}

	m.Timestamp = time.Now().Unix()
	return m
}

func report(url, key string, hostInfo model.HostInfo, metrics model.Metrics) {
	payload := model.AgentReport{
		SecretKey: key,
		Host:      hostInfo,
		Metrics:   metrics,
	}

	body, err := json.Marshal(payload)
	if err != nil {
		log.Printf("marshal error: %v", err)
		return
	}

	resp, err := http.Post(url, "application/json", bytes.NewReader(body))
	if err != nil {
		log.Printf("report error: %v", err)
		return
	}
	defer resp.Body.Close()

	if resp.StatusCode != 200 {
		var result model.Response
		json.NewDecoder(resp.Body).Decode(&result)
		log.Printf("report failed: %d - %s", resp.StatusCode, result.Message)
	}
}
