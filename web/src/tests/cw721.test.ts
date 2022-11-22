import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { Token } from "../logic/types";
import { Cw721Contract } from "./../logic/cw721";

const signingClient = {
  instantiate: jest.fn(),
  execute: jest.fn(),
  queryContractSmart: jest.fn(),
};

const RECEIVER = "receiver";
const TOKEN_ID = "1";
const MINTER = "minter";
const TOKEN_URI = "ipfs12345";
const CONTRACT = "wasm12345";
const RETURNED_HASH = { transactionHash: "123" };

describe("cw721", () => {
  const cw721 = new Cw721Contract(
    CONTRACT,
    signingClient as unknown as SigningCosmWasmClient
  );

  it("should proceed instantiation", async () => {
    const codeId = 123;
    const returnValue = { contractAddress: "address" };
    const initSpy = jest.spyOn(Cw721Contract, "instantiate");
    signingClient.instantiate.mockReturnValue(returnValue);

    Cw721Contract.instantiate(
      MINTER,
      signingClient as unknown as SigningCosmWasmClient,
      codeId
    );

    expect(signingClient.instantiate).toHaveBeenCalled();
    expect(signingClient.instantiate).toHaveBeenCalledWith(
      MINTER,
      codeId,
      { minter: MINTER, name: expect.any(String), symbol: expect.any(String) },
      expect.any(String),
      {
        amount: [{ amount: expect.any(String), denom: expect.any(String) }],
        gas: expect.any(String),
      }
    );
    expect(signingClient.instantiate).toHaveReturnedWith(returnValue);
    expect(initSpy).toHaveBeenCalled();
    expect(initSpy).toHaveBeenCalledWith(MINTER, signingClient, codeId);
  });

  it("should proceed minting", async () => {
    signingClient.execute.mockReturnValue(RETURNED_HASH);

    cw721.mintToken(MINTER, RECEIVER, TOKEN_ID, TOKEN_URI);

    expect(signingClient.execute).toHaveBeenCalled();
    expect(signingClient.execute).toHaveBeenCalledWith(
      MINTER,
      CONTRACT,
      {
        mint: {
          token: { owner: RECEIVER, token_id: TOKEN_ID, token_uri: TOKEN_URI },
        },
      },
      {
        amount: [{ amount: expect.any(String), denom: expect.any(String) }],
        gas: expect.any(String),
      }
    );
    expect(signingClient.execute).toHaveReturnedWith(RETURNED_HASH);
  });

  it("should proceed transferring", async () => {
    signingClient.execute.mockReturnValue(RETURNED_HASH);

    cw721.transferToken(MINTER, TOKEN_ID, RECEIVER);

    expect(signingClient.execute).toHaveBeenCalled();
    expect(signingClient.execute).toHaveBeenCalledWith(
      MINTER,
      CONTRACT,
      {
        transfer_nft: {
          to: RECEIVER,
          token_id: TOKEN_ID,
        },
      },
      {
        amount: [{ amount: expect.any(String), denom: expect.any(String) }],
        gas: expect.any(String),
      }
    );
    expect(signingClient.execute).toHaveReturnedWith(RETURNED_HASH);
  });

  it("should proceed querying tokens", async () => {
    const returnValue: Token[] = [
      { owner: MINTER, token_id: TOKEN_ID, token_uri: TOKEN_URI },
      { owner: MINTER, token_id: "2", token_uri: "ipfs54321" },
    ];
    signingClient.queryContractSmart.mockReturnValue(returnValue);

    cw721.getOwnedTokens(MINTER);

    expect(signingClient.queryContractSmart).toHaveBeenCalled();
    expect(signingClient.queryContractSmart).toHaveBeenCalledWith(CONTRACT, {
      tokens: { owner: MINTER },
    });
    expect(signingClient.queryContractSmart).toHaveReturnedWith(returnValue);
  });
});
