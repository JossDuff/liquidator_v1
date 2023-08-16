# liquidator_v1


# TODO
 - maybe subgraph query: 
// https://github.com/graphql-rust/juniper/issues/429
// https://petkopavlovski-42828.medium.com/blazingly-fast-yet-simple-graphql-client-written-in-rust-%EF%B8%8F-f608c8c38702

# Roadmap

## Phase 1: Start
- extremely basic liquidator
- troll chain since protocol creation and find market enter/exits
    - start a process for present and past blocks
- call to liquidator smart contract

## Phase 2: Correct
- proper error handling
- multicall getAccountLiquidity

## Phase 3: Scale
- async tokio for waiting for I/O interactions, rayon for speeding up computations
    - https://tokio.rs/tokio/tutorial
    - https://docs.rs/rayon/latest/rayon/
    - recusive async historical event query instead of block stepping (read async book)
- using get_logs or get_logs_paginated might be more efficient than query
- Determine smallest accurate amount of current users.  Phase 1 just took all users who entered a market, but we could also maybe use marketExited to find out which users are still around
- store positions in a database
- multithread
- batching RPC calls (multicall)
- websocket instead of https for performance (maybe no.  End goal is to be on IPC anyways)
- Confirm the fastest way to get chain data: API, theGraph, or custom indexer

## Phase 4: Spread
- script to deploy onto any lending protocol and fork
- function to find the comptroller contract creation block for each protocol
- target small lending protocols with not much competition

## Optional Phase: Data
- dune dashboard to track stats from all my deployed bots

## Phase 5: Slaughter
- suit up to be competive on large lending protocols
- my own ethereum/OP/ARB/etc nodes
- switch to IPC for even faster speeds
    - https://www.gakonst.com/ethers-rs/providers/ipc.html
- private flashbot mempool submitting transactions directly to miners
    - https://docs.flashbots.net/

## Refactor to alloy when alloy is done

To research: On L2s where gas is very cheap, constantly create new accounts and call entermarket call.  On accounts creation send address to bot and bot filters out that address.  Possibly clogs up competitor liquidators.  Might not be worth it for the gas fees and good liquidation bots might have a safeguard against this.

# Sources
 - https://www.comp.xyz/t/the-compound-iii-liquidation-guide/3452