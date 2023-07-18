package main

/*
#include <stdlib.h>
*/
import "C"
import (
	"github.com/google/go-containerregistry/pkg/authn"
	"github.com/google/go-containerregistry/pkg/crane"
	"github.com/google/go-containerregistry/pkg/v1"
	"unsafe"
)

func makeOption(user string, password string) crane.Option {
	return func(opts *crane.Options) {
		crane.WithPlatform(&v1.Platform{OS: "linux", Architecture: "amd64"})(opts)

		auth := authn.Anonymous
		if user != "" && password != "" {
			auth = authn.FromConfig(authn.AuthConfig{Username: user, Password: password})
		}

		crane.WithAuth(auth)(opts)
		opts.Keychain = nil
	}
}

//export ImageInspect
func ImageInspect(image_cstr *C.char, user_cstr *C.char, password_cstr *C.char) (bool, *C.char) {
	image := C.GoString(image_cstr)
	user := C.GoString(user_cstr)
	password := C.GoString(password_cstr)

	option := makeOption(user, password)
	json, err := crane.Config(image, option)

	if err != nil {
		return true, C.CString(err.Error())
	}
	return false, C.CString(string(json))
}

//export GoFree
func GoFree(ptr *C.char) {
	C.free(unsafe.Pointer(ptr))
}

func main() {}
