/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_DFX_NETWORK: string
  readonly VITE_AUTH_CANISTER_ID: string
  readonly VITE_BACKEND_CANISTER_ID: string
  readonly VITE_NFT_CANISTER_ID: string
  readonly VITE_INTERNET_IDENTITY_URL: string
  readonly VITE_IC_HOST: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}