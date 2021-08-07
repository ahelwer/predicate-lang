// Wasmer will let you easily run Wasm module in a Rust host.
//
// This example illustrates the basics of using Wasmer through a "Hello World"-like project:
//
//   1. How to load a Wasm modules as bytes
//   2. How to compile the module
//   3. How to create an instance of the module
//
// You can run the example directly by executing in Wasmer root:
//
// ```shell
// go test examples/example_instance_test.go
// ```
//
// Ready?

package main

import (
	"fmt"
	"io/ioutil"
	"time"

	"github.com/gravitational/acre/pkg/types"
	"github.com/wasmerio/wasmer-go/wasmer"
)

func ExampleInstance() {
	wasmBytes, err := ioutil.ReadFile("target/wasm32-unknown-unknown/debug/predicate.wasm")
	if err != nil {
		panic(err)
	}

	fmt.Println("Compiling module...")
	module, err := wasmer.NewModule(wasmer.NewStore(wasmer.NewEngine()), wasmBytes)

	if err != nil {
		fmt.Println("Failed to compile module:", err)
	}

	fmt.Println("Instantiating module...")
	// Let's instantiate the Wasm module.
	instance, err := wasmer.NewInstance(module, wasmer.NewImportObject())

	if err != nil {
		panic(fmt.Sprintln("Failed to instantiate the module:", err))
	}

	add, err := instance.Exports.GetFunction("add")

	if err != nil {
		panic(fmt.Sprintln("Failed to get the `add` function:", err))
	}

	fmt.Println("Calling `add` function...")
	result, err := add(1, 2)

	if err != nil {
		panic(fmt.Sprintln("Failed to call the `add` function:", err))
	}

	fmt.Println("Results of `add`:", result)

	w := createInstanceAllocator(instance)

	start := time.Now()
	iters := 13000
	for i := 0; i < iters; i++ {
		message := &types.Request{
			Message: "Hello, World!",
		}

		ptrVal, len := writeMessage(w, message)

		rePtrVal, err := w.call(ptrVal, len)
		if err != nil {
			panic(err)
		}

		data, err := readMessageBytes(w, rePtrVal)
		if err != nil {
			panic(err)
		}

		w.dealloc(rePtrVal)

		var response types.Response
		response.Unmarshal(data)
		// fmt.Printf("Response: %#v", response)
	}
	diff := time.Since(start)
	fmt.Printf("%v iterations in %v, %v per iteration", iters, diff, diff/time.Duration(iters))
}

func createInstanceAllocator(instance *wasmer.Instance) wasm {
	w := wasm{}
	var err error

	w.alloc, err = instance.Exports.GetFunction("alloc")
	if err != nil {
		panic(err)
	}

	w.dealloc, err = instance.Exports.GetFunction("dealloc")
	if err != nil {
		panic(err)
	}

	w.setAt, err = instance.Exports.GetFunction("set_at")
	if err != nil {
		panic(err)
	}

	w.getAt, err = instance.Exports.GetFunction("get_at")
	if err != nil {
		panic(err)
	}

	w.call, err = instance.Exports.GetFunction("call")
	if err != nil {
		panic(err)
	}

	w.getAllocSize, err = instance.Exports.GetFunction("get_alloc_size")
	if err != nil {
		panic(err)
	}
	return w
}

type wasm struct {
	alloc        func(...interface{}) (interface{}, error)
	dealloc      func(...interface{}) (interface{}, error)
	setAt        func(...interface{}) (interface{}, error)
	getAt        func(...interface{}) (interface{}, error)
	getAllocSize func(...interface{}) (interface{}, error)
	call         func(...interface{}) (interface{}, error)
}

type m interface {
	MarshalTo(dAtA []byte) (int, error)
	Size() (n int)
}

func marshalMessage(msg m) []byte {
	messageSize := msg.Size()
	bytes := make([]byte, messageSize)
	_, err := msg.MarshalTo(bytes)
	if err != nil {
		panic(err)
	}
	return bytes
}

func readMessageBytes(w wasm, ptrVal interface{}) ([]byte, error) {
	size, _ := w.getAllocSize(ptrVal)
	data := make([]byte, size.(int32))
	for i := range data {
		b, err := w.getAt(ptrVal, i)
		if err != nil {
			panic(err)
		}
		data[i] = byte(b.(int32))
	}
	return data, nil
}

func writeMessage(w wasm, msg m) (interface{}, int) {
	data := marshalMessage(msg)

	ptrVal, err := w.alloc(len(data))
	//	fmt.Printf("allocated: %v for data(%v) %#v\n", ptrVal, len(data), data)
	if err != nil {
		panic(err)
	}

	for i, byte := range data {
		_, err = w.setAt(ptrVal, i, int32(byte))
		if err != nil {
			panic(err)
		}
	}

	return ptrVal, len(data)
}

func main() {
	ExampleInstance()
}
