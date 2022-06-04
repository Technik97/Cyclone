<script lang="ts">
  import { sign_up } from "../data/sign_up";

  let username; let email; let password;

  const register = async () => {
    await sign_up(username, email, password);
  }
</script>

<div class="min-h-screen flex flex-col">
            <div class="container max-w-sm mx-auto flex-1 flex flex-col items-center justify-center px-2 ">
                <div class="bg-dark-blue px-6 py-8 rounded shadow-md text-black w-full">
                  <h1 class="mb-8 text-3xl text-center">Sign up</h1>
                    <form on:submit|preventDefault="{register}">
                      <input
                          bind:value={username}
                          type="text"
                          class="block border bg-grey-500 border-grey-light w-full p-3 rounded mb-4"
                          name="fullname"
                          placeholder="Full Name"
                          required />
                      <input
                          bind:value={email}
                          type="email"
                          class="block border border-grey-light w-full p-3 rounded mb-4"
                          name="email"
                          placeholder="Email"
                          required />
                      <input
                          bind:value={password}
                          type="password"
                          class="block border border-grey-light w-full p-3 rounded mb-4"
                          name="password"
                          placeholder="Password"
                          required />
                      <button
                          type="submit"
                          class="w-full bg-lime-500 px-4 py-3 border border-fuchsia-300 rounded hover:bg-lime-800 focus:outline-none my-1"
                      >Create Account</button>
                    </form>
                </div>

                <div class="text-grey-dark mt-6">
                    Already have an account?
                    <a class="no-underline border-b border-blue text-blue" href="../login/">
                        Log in
                    </a>.
                </div>
            </div>
        </div>
