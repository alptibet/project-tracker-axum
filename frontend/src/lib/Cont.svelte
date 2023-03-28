<script lang="ts">
  import axios from 'axios';
  import type { Contractor } from '../types';
  const cont = async () => {
    try {
      const res: Contractor = await axios({
        method: 'get',
        url: 'http://localhost:3000/api/v1/contractors',
      });
      return res;
    } catch (error) {
      alert(error.response.data.message);
    }
  };
</script>

<button on:click={cont}> CONT </button>
