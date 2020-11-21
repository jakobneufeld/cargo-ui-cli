<template>
  <v-app>
    <v-app-bar
      app
      color="primary"
      dark
    >
        <span class="-2"> Cargo UI </span>
    </v-app-bar>
    <v-main>
      <v-row justify="center">
    <v-dialog
      v-model="dialog"
      persistent
      max-width="600px"
    >
      <template v-slot:activator="{ on, attrs }">
        <v-btn
          color="primary"
          dark
          v-bind="attrs"
          v-on="on"
        >
        Add Package
        </v-btn>
      </template>
      <v-card>
         <v-card-title>ADD PKGS FORM</v-card-title>
          <v-text-field
            v-model="addPackageName"
            label="Name Here...."
            clearable
          ></v-text-field>
          <v-btn @click="submit">Close and Submit</v-btn>
      </v-card>
    </v-dialog>
  </v-row>
  <v-list>
    <div v-for="dep in dependencies" :key="dep">
      <v-list-item v-for="(name, version) in dep" :key="version">
        <v-list-item-avatar>
          <v-icon
            class="red lighten-1"
            dark
          >
            mdi-package-variant
          </v-icon>
        </v-list-item-avatar>
        <v-list-item-content>
          <v-list-item-title v-text="version"></v-list-item-title>
          <v-list-item-subtitle v-text="name"></v-list-item-subtitle>
        </v-list-item-content>
        <v-list-item-action>
         <v-dialog
      v-model="dialog2"
      persistent
      max-width="600px"
    >
      <v-card>
         <v-card-title>EDIT PKGS FORM</v-card-title>
          <v-text-field
            v-model="editversion"
            label="Verison Here...."
            clearable
          ></v-text-field>
          <v-btn @click="edit(version,editversion)">Close and Submit</v-btn>
      </v-card>
      <template v-slot:activator="{ on, attrs }">
        <v-btn
          color="primary"
          dark
          v-bind="attrs"
          v-on="on"
        >
        EDIT Package Verison
        </v-btn>
      </template>
    </v-dialog>
        </v-list-item-action>
      </v-list-item>
    </div>
</v-list>
    </v-main>
  </v-app>
</template>

<script>
import Axios from 'axios';

export default {
  name: 'App',
  components: {
  },
  data: () => ({
    dependencies: [],
    dialog: false,
    addPackageName: '',
    editversion: '',
    dialog2: false,
  }),
  created() {
    const localhostapi = Axios.create({
      baseURL: 'http://localhost:8000',
    });
    localhostapi.get('/readpkg').then((result) => this.setDependecies(result));
  },
  methods: {
    submit() {
      console.log('Hey');
      const localhostapi = Axios.create({
        baseURL: 'http://localhost:8000',
      });
      localhostapi.get(this.addPackageUrl).catch((error) => console.log('eror was', error));
      this.dialog = false;
      localhostapi.get('/readpkg').then((result) => this.setDependecies(result));
    },
    setDependecies(result) {
      this.dependencies = result.data;
    },
    edit(pkgname, version) {
      console.log('Hey');
      const localhostapi = Axios.create({
        baseURL: 'http://localhost:8000',
      });
      localhostapi.get(`/editpkg/${pkgname}/${version}`);
      this.dialog2 = false;
    },
    show() {
      this.dialog2 = true;
    },
  },
  computed: {
    addPackageUrl() {
      const url = `/addpkg/${this.addPackageName}`;
      return url;
    },
  },
};
</script>
