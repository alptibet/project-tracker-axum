<script lang="ts">
  import axios from 'axios';
  const getContractors = async () => {
    try {
      const res = await axios({
        method: 'get',
        url: 'http://localhost:3000/api/v1/contractors',
        withCredentials: true,
      });
    } catch (error) {
      alert(error.response.data.message);
    }
  };
</script>

<button on:click={getContractors}> CONT </button>
