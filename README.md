# liquidator_v1

# Roadmap

## Phase 1: Start
- extremely basic liquidator
- API call and flash swap

## Phase 2: Scale
- tokio for waiting for I/O interactions, rayon for speeding up computations
    - https://tokio.rs/tokio/tutorial
    - https://docs.rs/rayon/latest/rayon/
- store positions in a database
- batching RPC calls (multicall)
- websocket instead of https for performance (maybe no.  End goal is to be on IPC anyways)
- Confirm the fastest way to get chain data: API, theGraph, or custom indexer

## Phase 2: Spread
- script to deploy onto any lending protocol and fork
- target small lending protocols with not much competition

## Phase 3: Slaughter
- suit up to be competive on large lending protocols
- my own ethereum/OP/ARB/etc nodes
- switch to IPC for even faster speeds
    - https://www.gakonst.com/ethers-rs/providers/ipc.html
- private flashbot mempool submitting transactions directly to miners
    - https://docs.flashbots.net/


# Sources
 - https://www.comp.xyz/t/the-compound-iii-liquidation-guide/3452