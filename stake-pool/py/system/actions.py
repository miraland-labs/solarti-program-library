from miraland.publickey import PublicKey
from miraland.rpc.async_api import AsyncClient
from miraland.rpc.commitment import Confirmed


async def airdrop(client: AsyncClient, receiver: PublicKey, lamports: int):
    print(f"Airdropping {lamports} lamports to {receiver}...")
    resp = await client.request_airdrop(receiver, lamports, Confirmed)
    await client.confirm_transaction(resp['result'], Confirmed)
