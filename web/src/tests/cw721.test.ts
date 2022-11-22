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
const RETURNED_CALL_RESULT = Promise.resolve({
  transactionHash: "123",
});
const DEFAULT_FUNDS = {
  amount: [{ amount: expect.any(String), denom: expect.any(String) }],
  gas: expect.any(String),
};

describe("cw721", () => {
  const cw721 = new Cw721Contract(
    CONTRACT,
    signingClient as unknown as SigningCosmWasmClient
  );

  it("should proceed instantiation", async () => {
    const codeId = 123;
    const returnValue = Promise.resolve({ contractAddress: "address" });
    const initSpy = jest.spyOn(Cw721Contract, "instantiate");
    signingClient.instantiate.mockReturnValue(returnValue);

    const initRes = await Cw721Contract.instantiate(
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
      DEFAULT_FUNDS
    );
    expect(signingClient.instantiate).toHaveReturnedWith(returnValue);
    expect(initSpy).toHaveBeenCalled();
    expect(initSpy).toHaveBeenCalledWith(MINTER, signingClient, codeId);
    expect(initRes).toEqual((await returnValue).contractAddress);
  });

  it("should proceed minting", async () => {
    const mintSpy = jest.spyOn(cw721, "mintToken");
    signingClient.execute.mockReturnValue(RETURNED_CALL_RESULT);

    const mintTx = await cw721.mintToken(MINTER, RECEIVER, TOKEN_ID, TOKEN_URI);

    expect(signingClient.execute).toHaveBeenCalled();
    expect(signingClient.execute).toHaveBeenCalledWith(
      MINTER,
      CONTRACT,
      {
        mint: {
          token: { owner: RECEIVER, token_id: TOKEN_ID, token_uri: TOKEN_URI },
        },
      },
      DEFAULT_FUNDS
    );
    expect(signingClient.execute).toHaveReturnedWith(RETURNED_CALL_RESULT);
    expect(mintSpy).toHaveBeenCalled();
    expect(mintSpy).toHaveBeenCalledWith(MINTER, RECEIVER, TOKEN_ID, TOKEN_URI);
    expect(mintTx).toEqual((await RETURNED_CALL_RESULT).transactionHash);
  });

  it("should proceed transferring", async () => {
    const transferSpy = jest.spyOn(cw721, "transferToken");
    signingClient.execute.mockReturnValue(RETURNED_CALL_RESULT);

    const transferTx = await cw721.transferToken(MINTER, TOKEN_ID, RECEIVER);

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
      DEFAULT_FUNDS
    );
    expect(signingClient.execute).toHaveReturnedWith(RETURNED_CALL_RESULT);
    expect(transferSpy).toHaveBeenCalled();
    expect(transferSpy).toHaveBeenCalledWith(MINTER, TOKEN_ID, RECEIVER);
    expect(transferTx).toEqual((await RETURNED_CALL_RESULT).transactionHash);
  });

  it("should proceed querying tokens", async () => {
    const getTokensSpy = jest.spyOn(cw721, "getOwnedTokens");
    const returnValue: Promise<Token[]> = Promise.resolve([
      { owner: MINTER, token_id: TOKEN_ID, token_uri: TOKEN_URI },
      { owner: MINTER, token_id: "2", token_uri: "ipfs54321" },
    ]);
    signingClient.queryContractSmart.mockReturnValue(returnValue);

    const tokens = await cw721.getOwnedTokens(MINTER);

    expect(signingClient.queryContractSmart).toHaveBeenCalled();
    expect(signingClient.queryContractSmart).toHaveBeenCalledWith(CONTRACT, {
      tokens: { owner: MINTER },
    });
    expect(signingClient.queryContractSmart).toHaveReturnedWith(returnValue);
    expect(getTokensSpy).toHaveBeenCalled();
    expect(getTokensSpy).toHaveBeenCalledWith(MINTER);
    expect(tokens).toEqual(await returnValue);
  });
});
