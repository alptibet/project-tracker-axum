<script lang="ts">
  import axios from 'axios';
  import type { UserLogin } from '../types';

  const login = async () => {
    const data: UserLogin = {
      username: 'alptibet',
      password: '12345678',
    };
    try {
      const res = await axios({
        method: 'post',
        url: 'http://localhost:3000/api/v1/login',
        data,
        withCredentials: true,
      });
    } catch (error) {
      alert(error.response.data.message);
    }
  };
</script>

<button on:click={login}> LOGIN </button>
