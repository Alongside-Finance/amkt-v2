package client

const (
	// 0,1,2 used for stdin,stdout,stderr
	HClientRFd = iota + 3
	HClientWFd
	PClientRFd
	PClientWFd
	MaxFd
)
