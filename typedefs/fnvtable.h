struct FnVtable {
    void* drop_in_place;     // offset 0x00
    unsigned long long size; // offset 0x08
    unsigned long long align;// offset 0x10
    void* call;              // offset 0x18 (Fn::call)
    void* call_mut;          // offset 0x20 (FnMut::call_mut)
    void* call_once;         // offset 0x28 (FnOnce::call_once)
};

struct RustClosureVTable {
    void* call;     // pointer to closure function
    void* drop;               // pointer to destructor for environment
    unsigned long long size;              // size of environment
    unsigned long long align;             // alignment of environment
};

struct FnTraitObject {
    void* data;              // 0x00 (closure environment)
    void* vtable;            // 0x08 (pointer to FnVtable)
};