<template>
  <tr>
    <td>{{ student.id }}</td>
    <td>{{ student.name }}</td>
    <td>{{ student.roll_no }}</td>
    <td>{{ student.attendance }}</td>
  </tr>
</template>

<script>
export default {
  props: {
    student: {
      type: Object
    }
  }
}
</script>
