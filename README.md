# PDA through CPI 🦀⚓️

[PDAs](https://book.anchor-lang.com/anchor_in_depth/PDAs.html) (Program Derived Addresses) and [CPIs](https://book.anchor-lang.com/anchor_in_depth/CPIs.html) (Cross-Program Invocations) are two topics which are incredibly important for writing non-trivial Solana programs. While these topics are covered extensively independently, there is currently (mid-2022) relatively limited coverage of how to use these two concepts together.

This repo aims to provide a minimal example of how to do a CPI where the program that is being invoked stores/writes some data in a PDA.

### What are PDAs?

A PDA is a special kind of address that is used to store data. A PDA looks like a public key (like a wallet) but it is cryptographically impossible for there to be a private key to this address. As such, these addresses can be used to store data that is not necessarily associated with a user.

For more information on PDAs, see [the "PDAs" section of the Anchor Book](https://book.anchor-lang.com/anchor_in_depth/PDAs.html).

### What are CPIs?

When one program wants to call a method of another program, it must use what is called a Cross-Program Invocation (or CPI). Practically, using CPIs requires the developer to construct a reference to the program that needs to be called, the accounts that this program will use, and the data that needs to be passed to the program.

For more information on CPIs, see [the "CPIs" section of the Anchor Book](https://book.anchor-lang.com/anchor_in_depth/CPIs.html).

### Wait, what do you mean "accounts that this program will use"?

Good catch! As you may know, when a Solana program is invoked, the client must articulate up-front which accounts the program will use. The Solana runtime uses this information to parallelise transactions.

You may be familiar with code that looks like this:

```js
await myProgram.methods
    .myMethod()
    .accounts({
        foo: fooAccount,
        bar: barAccount,
    })
    .rpc();
```

In this case, the `myProgram.methods.myMethod()` call will read data from the `fooAccount` and `barAccount` accounts while executing the transaction. As such, we need to articulate this up-front so that Solana can make sure no other transactions in the block will use the same accounts.

### OK, what if I CPI a method which uses a PDA?

Let's imagine that you have two programs: manager and worker. 

Consider that the worker program has a method called `set_data` which edits an account called `data`. When you `set_data` directly on the worker program, this would look something like this:

```js
await worker.methods
    .setData(new anchor.BN(1337))
    .accounts({
        data: dataPDA,
    })
    .rpc();
```

Similarly, if you want to call a method on the manager program which would then in-turn call the `set_data` method of the worker program via CPI, you need to also include the `data` account in the accounts list (in addition to the address of the worker program itself). This might look something like this:

```js
await manager.methods
    .setWorkerDataThroughCpi(new anchor.BN(42))
    .accounts({
        workerProgram: worker.programId,
        data: dataPDA,
    })
    .rpc();
```

Even if the manager program *could* `find_program_address` the address of the data PDA itself, it must be articulated up-front by the client!

### Can I see a working example?

Yes! This repo contains a working example of both the worker program and the manager program! As discussed, the [worker program](./programs/worker/src/lib.rs) is a simple program that simply writes a value to an account. The [manager program](./programs/manager/src/lib.rs) is a simple program that simply calls the worker program. You can then see how to invoke these programs by reading [the tests](./tests/manager.ts)!

### Conclusion

If you're just getting started with PDAs and CPIs, I hope this helps! 🚀