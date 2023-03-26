<script lang="ts">
  import axios from 'axios';
  const login = async () => {
    try {
      await axios({
        method: 'post',
        url: 'http://localhost:3000/api/v1/users/login',
        data: {
          username: 'alptibet',
          password: '12345678',
        },
      });
      // if (res.data.status === 'success') {
      //   alert('Loggin in');
      //   window.setTimeout(() => {
      //     location.assign('/overview'), 1500;
      //   });
      // }
    } catch (error) {
      alert(error.response.data.message);
    }
  };
</script>

<button on:click={login}> LOGIN </button>
