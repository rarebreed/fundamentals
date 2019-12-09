//! Stack data structure from leetcode
//! Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.
//! 
//! So, this sounds a little like a trick question.  At first glance, it seems like an array or vector would fit the
//! bill, with perhaps an extra field that always contains the minimum value, such that element 0 is always the second
//! highest value.  However, there's a slight problem with this.  In theory, as we grow the elements in the stack, we
//! may need to reallocate memory to increase the size of the array or vector.  In a worst case scenario, this means
//! that we need to move N elements in the vector to the newly grown vector.
//! 
//! Since the question only says that push, pop, and top must be in constant time, it actually makes more sense to use
//! a linked list for this data structure, where the head always contains the minimum value.  This structure will also
//! give constant time access for a push, pop and top as desired
