include wasm/Wasm.mk

v-dev:
	pnpm vercel -- dev

v-deploy:
	pnpm vercel -- deploy
