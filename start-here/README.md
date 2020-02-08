CMake will chain-call cargo as needed.
For tasks.json to work:

From a 64-bit VS2019 developer prompt:
```
mkdir out/build/x64-Release
cd out/build/x64-Release
cmake ../../.. -DCMAKE_BUILD_TYPE=Release -G Ninja
ninja
```

This should format the code, restore cargo crates, build the rust library, build the c++ executable, then run it :)
