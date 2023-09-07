package main

/*
#include <stdlib.h>

struct ImageMetadataReturn {
	char *error;
	char *config;
	char *digest;
	char *manifest;
};
*/
import "C"
import (
	"github.com/google/go-containerregistry/pkg/authn"
	"github.com/google/go-containerregistry/pkg/crane"
	"github.com/google/go-containerregistry/pkg/name"
	"github.com/google/go-containerregistry/pkg/v1"
	"runtime/debug"
	"unsafe"
)

//export GetBuildInfo
func GetBuildInfo() *C.char {
	buildinfo, ok := debug.ReadBuildInfo()
	var buildinfostr = ""
	if ok {
		buildinfostr = buildinfo.String()
	}

	return C.CString(buildinfostr)
}

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

// nginx -> index.docker.io/library/nginx@sha256:48a84a0728cab8ac558f48796f901f6d31d287101bc8b317683678125e0d2d35
func getDigest(image string, option crane.Option) (string, error) {
	digest, err := crane.Digest(image, option)
	if err != nil {
		return "", err
	}

	ref, err := name.ParseReference(image)
	if err != nil {
		return "", err
	}

	ret := ref.Context().Digest(digest).String()
	return ret, nil
}

//export ImageMetadata
func ImageMetadata(image_cstr *C.char, user_cstr *C.char, password_cstr *C.char) C.struct_ImageMetadataReturn {
	image := C.GoString(image_cstr)
	user := C.GoString(user_cstr)
	password := C.GoString(password_cstr)

	option := makeOption(user, password)

	ret := C.struct_ImageMetadataReturn{}

	digest, err := getDigest(image, option)
	if err != nil {
		ret.error = C.CString(err.Error())
		return ret
	}
	ret.digest = C.CString(digest)

	config, err := crane.Config(digest, option)
	if err != nil {
		ret.error = C.CString(err.Error())
		return ret
	}
	ret.config = C.CString(string(config))

	manifest, err := crane.Manifest(digest, option)
	if err != nil {
		ret.error = C.CString(err.Error())
		return ret
	}
	ret.manifest = C.CString(string(manifest))

	return ret
}

//export FreeImageMetadataReturn
func FreeImageMetadataReturn(ret C.struct_ImageMetadataReturn) {
	FreeStr(ret.error)
	FreeStr(ret.config)
	FreeStr(ret.manifest)
	FreeStr(ret.digest)
}

//export FreeStr
func FreeStr(ptr *C.char) {
	if ptr != nil {
		C.free(unsafe.Pointer(ptr))
	}
}

func main() {}
