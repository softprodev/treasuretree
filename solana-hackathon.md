Since trying out Rust programming on several other Rust blockchains,
we've been looking forward to test-driving Solana.
With Solana hosting a hackathon during May,
it was a good opportunity to give it a try.


- [The hackathon doc page][hackdocs]


[hackdocs]: https://github.com/solana-labs/solana-season



## Today's plan

Today is the 17th.
The hackathon began on the 15th,
but we're just starting now after moving Airbnbs.

Our plan today is to join the Solana hackathon Discord channels,
install the Solana SDK,
and run some hello world example.

While we get started,
we open [one of the video sessions we missed][tw],
this one an intro to "the Solana programming model".

[tw]: https://www.twitch.tv/videos/1021435633

We learn

- that "everything is an account"
- storage costs rent
- accounts can be pre-paid for 2 years
- contracts don't hold state (not sure what that means yet)
- at least some tokens don't require creating new contracts
- program-derived accounts don't have a knowable private key,
  but can still sign transactions (or something).
  This sounds like it solves a problem I have been having
  understanding how to build distributed applications,
  but I'll have to investigate further.
- the [solana-program-library] is something I should get to know.

[solana-program-library]: https://github.com/solana-labs/solana-program-library


## Installing the Solana SDK

I want to install the Solana SDK, tools, or whatever I need.
I assume there's something SDK-like I need.

The [hackathon docs][hackdocs] contains some educational links,
and I folow the firt to the [Solana docs website][soldocs].

[soldocs]: https://docs.solana.com/

The first link is ["Start Building"][start],
and that sounds good.

[start]: https://docs.solana.com/developing/programming-model/overview

This page says "To start developing immediately you can build, deploy, and run one of the examples."
That's exactly what we want to do.

So now we're following the [helloworld example][he] docs.

[he]: https://docs.solana.com/developing/on-chain-programs/examples

I run

```
$ git clone https://github.com/solana-labs/example-helloworld.git
```

And am directed to continue by following the [hello-world README][hwrm],
so I do so.

[hwrm]: https://github.com/solana-labs/example-helloworld/blob/master/README.md

This flow is a bit confusing,
as the "examples" page proceeds directly from "Helloworld",
to "Break",
which is an entirely different example.
Though the instructions do say to continue to the Helloworld readme,
Aimee succombs to this confusion,
and proceeds to try to follow "Break".
The correct thing to do is stop here,
navigate to the Helloworld readme,
and continue there.

There are enough docs for this example,
that the README has a table of contents.
I am encouraged by this.

This requires node > 14.
Aimee immediately runs into problems on mac:
she seems to have node 10,
tries to upgrade with brew,
but brew complains because her node and yarn
are not from brew.
This happens all the time with her computer &mdash;
she has forgotten how she installed node last time.
We discover she has nvm installed and sort things out.
We're both on node v16.1.0.

We run these things:

```
$ nvm install stable
$ nvm use stable
$ rustup update stable
```

And now, "install Solana v1.6.6 or later",
the instructions for which are
[back on the Solana docs site][tooldocs].

[tooldocs]: https://docs.solana.com/cli/install-solana-cli-tools

I run

```
$ sh -c "$(curl -sSfL https://release.solana.com/v1.6.9/install)"
```


## Looking at the install script


While that is running,
I download and read the script,
which I [have gisted here][shgist].

[shgist]: https://gist.github.com/brson/29f82547df862161dda8cbc92de6ab37

I open it and am _so pleased_.

At the top I see:

```
# Copyright 2016 The Rust Project Developers. See the COPYRIGHT
# file at the top-level directory of this distribution and at
# http://rust-lang.org/COPYRIGHT.
```

and then

```
# This is just a little script that can be downloaded from the internet to
# install solana-install. It just does platform detection, downloads the installer
# and runs it.
```

This is a clone of [rustup's install script][rustup-init],
which I wrote,
and which,
[as I discovered previously][dfin],
Dfinity also copied.

[rustup-init]: https://github.com/rust-lang/rustup/blob/0eaf2f92b6016447210f2defccaf4bfcc1f4e9ff/rustup-init.sh
[dfin]: https://brson.github.io/2021/01/30/dfinity-impressions

I see a peculiar comment at the top:

```
{ # this ensures the entire script is downloaded #
```

TODO


## Running a devnet

The install succeeds and I have the Solana tools:

```
$ $ solana --version
solana-cli 1.6.9 (src:9e42883d; feat:2960423209)
```

I continue to configure the CLI per the Helloworld readme:

```
$ solana config set --url localhost
Config File: /home/brian/.config/solana/cli/config.yml
RPC URL: http://localhost:8899
WebSocket URL: ws://localhost:8900/ (computed)
Keypair Path: /home/brian/.config/solana/id.json
Commitment: confirmed
```

What does this tell us?

- We have a global config file,
  for me it's in the XDG-standard `~/.config/solana` directory,
- Solana's default RPC port is 8899,
- Nodes can also communicate over WebSockets
- We have an account keypair set up for us,
  the exact purpose for which is not clear,
  but I assume is a dev keypair
- "Commitment: confirmed" &mdash: I have no idea. Wat?

The next instruction is to create a keypair,
but I `solana config` says I already have a keypair.
I am guessing I have previously installed the Solana SDK,
and it is an old keypair.
I begin to generate a new one:

```
$ solana-keygen new
Generating a new keypair

For added security, enter a BIP39 passphrase

NOTE! This passphrase improves security of the recovery seed phrase NOT the
keypair file itself, which is stored as insecure plain text

BIP39 Passphrase (empty for none):
```

This is going to be a dev key and I don't care about recovering it.
I leave the passphrase blank and just hit "enter".

```
Wrote new keypair to /home/brian/.config/solana/id.json
==================================================================================
pubkey: C4NEJZc432PWEDvYR6LiBWCv7wvfdJWDppLscL42R3aD
==================================================================================
Save this seed phrase and your BIP39 passphrase to recover your new keypair:
engage possible prison twin control language talk cactus hobby vehicle allow blush
==================================================================================
```

Again, this is a garbage dev key.
I am fine printing the seed phrase on the internet.

I run a devnet node in one terminal:

```
$ solana-test-validator
Ledger location: test-ledger
Log: test-ledger/validator.log
Identity: GZr7zHFUxA7kjGgzUsUuRfQtNASBCGurynEg7yUDcfvP
Genesis Hash: F945qQyeHDUXN58eUWuLHLogAZ7Qgkpucc7xe8LisQnR
Version: 1.6.9
Shred Version: 54687
Gossip Address: 127.0.0.1:1025
TPU Address: 127.0.0.1:1027
JSON RPC URL: http://127.0.0.1:8899
⠒ 00:00:08 | Processed Slot: 16 | Confirmed Slot: 16 | Finalized Slot: 0 | Snapshot Slot: - | Transaction
```

And the log monitor in another:

```
$ solana logs
Streaming transaction logs. Confirmed commitment
```

Nother further happens.
Probably because there are no transactions on my devnet.

That's as far as I get for the night.

I like everything I'm seeing so far:
the docs have directed me well,
the tooling has worked cleanly.


## eBPF

After reading [a about Solana's use of BPF][bpfdoc],
I ask in their Discord a question that I have been wondering for a few weeks:

[bpfdoc]: https://docs.solana.com/developing/on-chain-programs/overview

> Can somebody from Solana remind me why Solana uses BPF as its instruction set?
  I thought it was because BPF was not turing complete and programs could be
  verified to terminate, but I've recently been informed that is not the case

Nobody answers,
but I may have asked in the wrong channel.

A ask again in #hack-blockchain-support:

> What advantages does solana get from targeting eBPF, vs any other instruction
  set, for its VM? Been trying to get an answer to this for awhile.

TODO


## The Helloworld application

It's 5/19 now.
[Next step][nshw] in the Helloworld tutorial is to build and run the application.
The build is driven by npm, but includes cargo components as well.

[nshw]: https://github.com/solana-labs/example-helloworld/blob/master/README.md#install-npm-dependencies

I run `npm install` and see this warning:

```
: ~/solana/example-helloworld
$ npm install
npm WARN EBADENGINE Unsupported engine {
npm WARN EBADENGINE   package: 'helloworld@0.0.1',
npm WARN EBADENGINE   required: { node: '12.x' },
npm WARN EBADENGINE   current: { node: 'v15.10.0', npm: '7.5.3' }
npm WARN EBADENGINE }
```

There is a problem here:
I have node v15, and the build gives me a warning that v12 is required,
but the readme for this demo says "v14 recommended".
So it appears that either the package should be updated to not warn on later versions,
or the docs should say "v12 recommended".

The install succeeds.

Next step is to build the Rust program:

```
$ npm run build:program-rust
```

And as part of this,
the build runs `cargo build-bpf`.
I am guessing the `build-bpf` subcommand was installed
during `npm install`,
which ran a cargo build.
`cargo build-bpf` downloads the Solana BPF SDK:

```
$ npm run build:program-rust
> helloworld@0.0.1 build:program-rust
> cargo build-bpf --manifest-path=./src/program-rust/Cargo.toml --bpf-out-dir=dist/program

BPF SDK: /home/brian/.local/share/solana/install/releases/1.6.9/solana-release/bin/sdk/bpf
```

I deploy the program:

```
$ solana program deploy dist/program/helloworld.so
Program Id: 2jW9jdWSwqkM2rznH5MwL65obzMoHUHZsxpUmjAULsmq
```

And the log fills up with transactions:

```
...
Transaction executed in slot 2006:
  Signature: 2jP1sCQ2CVzizHzc7zgajwf2E8yU2x4RhSyeL59mvFio8UvkyS4rcxyVaNyan5w7UCm3cZBsYefoETBD7DuZbbBt
  Status: Ok
  Log Messages:
    Program 11111111111111111111111111111111 invoke [1]
    Program 11111111111111111111111111111111 success
    Program 11111111111111111111111111111111 invoke [2]
    Program 11111111111111111111111111111111 success
    Deployed program 2jW9jdWSwqkM2rznH5MwL65obzMoHUHZsxpUmjAULsmq
```

I run the client JavaScript app,
and run into an error:

```
$ npm run start

> helloworld@0.0.1 start
> ts-node src/client/main.ts

node:internal/modules/cjs/loader:926
  throw err;
  ^

Error: Cannot find module 'arg'
Require stack:
- /home/brian/solana/example-helloworld/node_modules/ts-node/dist/bin.js
    at Function.Module._resolveFilename (node:internal/modules/cjs/loader:923:15)
    at Function.Module._load (node:internal/modules/cjs/loader:768:27)
    at Module.require (node:internal/modules/cjs/loader:995:19)
    at require (node:internal/modules/cjs/helpers:92:18)
    at Object.<anonymous> (/home/brian/solana/example-helloworld/node_modules/ts-node/dist/bin.js:8:13)
    at Module._compile (node:internal/modules/cjs/loader:1091:14)
    at Object.Module._extensions..js (node:internal/modules/cjs/loader:1120:10)
    at Module.load (node:internal/modules/cjs/loader:971:32)
    at Function.Module._load (node:internal/modules/cjs/loader:812:14)
    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:76:12) {
  code: 'MODULE_NOT_FOUND',
  requireStack: [
    '/home/brian/solana/example-helloworld/node_modules/ts-node/dist/bin.js'
  ]
}
npm ERR! code 1
npm ERR! path /home/brian/solana/example-helloworld
npm ERR! command failed
npm ERR! command sh -c ts-node src/client/main.ts

npm ERR! A complete log of this run can be found in:
npm ERR!     /home/brian/.npm/_logs/2021-05-19T01_48_31_041Z-debug.log
```

Hm, "cannot find module 'arg'".

I google "cannot find module arg".
Nothing.
I run `git pull` to check for bugfixes.
Nothing.
I see that I have some dirty files in my tree.
So I decide to start over,
run `git reset --hard origin/master && rm node_modules && rm dist`.

I go through the `npm install` / `npm run build:program-rust` / `solana program deploy` / `npm run start`
sequence again.

This time it works:

```
$ npm run start

> helloworld@0.0.1 start
> ts-node src/client/main.ts

Let's say hello to a Solana account...
Connection to cluster established: http://localhost:8899 { 'feature-set': 2960423209, 'solana-core': '1.6.9' }
Using account C4NEJZc432PWEDvYR6LiBWCv7wvfdJWDppLscL42R3aD containing 499999998.27048796 SOL to pay for fees
Using program 7NMkTRVNtBvuC68BVeamnXmuqcpiVeLQgNDGzQrPceTN
Creating account 6GeXM3KjbPJ7pXQoDri2f2YPzZXGts2ewHiVRFGuaZWt to say hello to
Saying hello to 6GeXM3KjbPJ7pXQoDri2f2YPzZXGts2ewHiVRFGuaZWt
6GeXM3KjbPJ7pXQoDri2f2YPzZXGts2ewHiVRFGuaZWt has been greeted 1 time(s)
Success
```

Ok.
I'll never know what I did wrong.
Love it when that happens,
but that's hacking.



## A look at the Rust contract

The Rust contract [is nice and short][hct].
Here's the whole thing minus imports and tests:

[hct]: https://github.com/solana-labs/example-helloworld/blob/master/src/program-rust/src/lib.rs

```rust
// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");

    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Increment and store the number of times the account has been greeted
    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
    greeting_account.counter += 1;
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Greeted {} time(s)!", greeting_account.counter);

    Ok(())
}
```

I like this a lot:
there's very little magic here &mdash;
you define an entry point,
and you get a blob of instruction data,
and its up to you to use the SDK to interpret it.
This contrasts with NEAR and Ink,
that both have complex macros that do a bunch of setup and dispatch before main.

Personally, I like to understand what is happening under the hood,
and dislike hiding magic in macros where reasonable,
so I'm encouraged by this.

This readme has [a section explaining what the program does][hwex]!

[hwex]: https://github.com/solana-labs/example-helloworld/blob/master/README.md#entrypoint

This is awesome.

We spend some time reading through the Rust and TypeScript source code on GitHub,
but I am not outlining our thoughts about it here.

All of the links to code in the "learn about the client" section of the readme
link to old commits,
and it is a bit confusing.

I [submit a PR][docpr] to update the docs.

[docpr]: https://github.com/solana-labs/example-helloworld/pull/217

I also discover that part of the `npm install` process runs `cargo update`,
which leaves my lockfile dirty.
I submit [a PR to update the lockfile][lockfile].

[lockfile]: https://github.com/solana-labs/example-helloworld/pull/216


## Integrating solana into our project

Now that we have the tools,
and a basic understanding of how to set up a Solana program and client,
let's think about integrating Solana into our own project.


### An overview of our project

It is called [geonft].
and it is a toy that connects NFTs to the physical world.
In it,

- treasure _planters_ physically plant QR codes containing secret keys,
- treasure _claimers_ find and scan those QR codes to claim them on the network, as NFTs.

As such, there are only two verbs in our app,
_plant_, and _claim_, and executing either
involves the creation and verification of a few cryptographic signatures.

[geonft]: https://github.com/brson/geonft

We have already prototyped the application as a conventional webapp using [Rocket].
Our goal for this hackathon is to implement the two verbs, plant and claim,
in a Solana program;
make the treasures transactable as NFTs;
and to create a service that syncs the state of these treasures
from the centralized service onto the blockchain.
