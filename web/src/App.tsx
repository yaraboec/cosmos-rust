import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { useEffect, useState } from "react";
import { config } from "./config";
import { Cw721Contract } from "./logic/cw721";
import { loadOrCreateWallet } from "./logic/wallet";
import { createSigningClient } from "./logic/client";
import { getErrorFromStackTrace } from "./logic/utils";

import "./App.css";

function App() {
  const [user, setUser] = useState<string>("");
  const [client, setClient] = useState<SigningCosmWasmClient | null>(null);
  const [cw721, setCw721] = useState<Cw721Contract | null>(null);
  const [error, setError] = useState<string>("");
  const [tokenId, setTokenId] = useState<string>("");
  const [transferAddress, setTransferAddress] = useState<string>("");
  const [tokensAddress, setTokensAddress] = useState<string>("");

  error && console.log(error);

  useEffect(() => {
    (async function setData() {
      try {
        const wallet = await loadOrCreateWallet(config.addressPrefix);
        const accounts = await wallet.getAccounts();

        console.log(accounts[0].address);
        setUser(accounts[0].address);

        const signingClient = await createSigningClient(config, wallet);
        setClient(signingClient);
        setCw721(
          new Cw721Contract(process.env.REACT_APP_CONTRACT!, signingClient!)
        );
      } catch (error: any) {
        setError(getErrorFromStackTrace(error));
      }
    })();
  }, []);

  const getTokens = async () => {
    const tokens = await cw721?.getOwnedTokens(tokensAddress);

    console.log(tokens);
  };

  const mintToken = async () => {
    const mint_res = await cw721?.mintToken(
      process.env.REACT_APP_MINTER!,
      transferAddress,
      tokenId
    );

    console.log(mint_res);
  };

  const transferToken = async () => {
    const mint_res = await cw721?.transferToken(
      tokensAddress,
      tokenId,
      transferAddress
    );

    console.log(mint_res);
  };

  return (
    <div className="App">
      {user && client && (
        <div className="manage">
          <span>
            address:
            <input onChange={(e) => setTokensAddress(e.target.value)}></input>
          </span>
          <button onClick={async () => await getTokens()}>see tokens</button>
          <span>
            receiver:
            <input onChange={(e) => setTransferAddress(e.target.value)}></input>
          </span>
          <span>
            id: <input onChange={(e) => setTokenId(e.target.value)}></input>
          </span>
          <button onClick={async () => await mintToken()}>mint</button>
          <span>
            from:
            <input onChange={(e) => setTokensAddress(e.target.value)}></input>
          </span>
          <span>
            to:
            <input onChange={(e) => setTransferAddress(e.target.value)}></input>
          </span>
          <span>
            id: <input onChange={(e) => setTokenId(e.target.value)}></input>
          </span>
          <button onClick={async () => await transferToken()}>transfer</button>
        </div>
      )}
    </div>
  );
}

export default App;
