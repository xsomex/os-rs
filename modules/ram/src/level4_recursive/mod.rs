// TODO: use paging (with 4 levels), with a recursive mapping of the whole physical memory.
// For each physical block, there is a counter (size: 1B?) which holds the number of times the
// block is referenced by a virtual block. It allows shared memory, or multi access to a memory
// mapped register.
// Maybe this should be an option (a bool for each block that says if it's mappable more than one
// times. The purpose is to know to how many pages is a frame mapped, without using too much
// memory, too much CPU or too much time.
// The method has to be well defined.
