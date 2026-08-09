import * as THREE from "three"
import { OBJLoader } from 'three/addons/loaders/OBJLoader.js';
import { MTLLoader } from 'three/addons/loaders/MTLLoader.js';

window.onload = () => {
    // Scene, camera, models, renderer
    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(90, window.innerWidth / window.innerHeight, 0.1, 10000);
    const renderer = new THREE.WebGLRenderer();
    let sailboat = undefined;

    scene.background = new THREE.Color("black");
    camera.position.z = 10;
    renderer.setSize(window.innerWidth, window.innerHeight);
    document.body.appendChild(renderer.domElement);
    
    // Lights
    const ambientLight = new THREE.AmbientLight(0xffffff, 1.5);
    const dirLight = new THREE.DirectionalLight(0xffffff, 2.5);
    
    dirLight.position.set(5, 10, 7.5);

    scene.add(ambientLight);
    scene.add(dirLight);

    // Model loaders
    const mtlLoader = new MTLLoader();
    const objLoader = new OBJLoader();
    
    // mtlLoader.load("assets/sailboat.mtl", (materials) => {
    //     materials.preload();
    //     objLoader.setMaterials(materials);
    //     objLoader.load("assets/sailboat.obj", (object) => {
    //         object.position.set(0, 0, 0);
    //         object.scale.setScalar(1);
    //         sailboat = object;

    //         scene.add(object);
    //     }, undefined, (error) => console.error(`OBJ error: ${error}`));
    // }, undefined, (error) => console.error(`MTL error: ${error}`));

    objLoader.load("assets/sailboat.obj", (object) => {
        object.position.set(0, 0, 0);
        object.scale.setScalar(1);
        sailboat = object;

        object.traverse((child) => {
            if (child.isMesh) {
                const edgesGeo = new THREE.EdgesGeometry(child.geometry);
                const lineMat = new THREE.LineBasicMaterial({ color: 0xffffff, transparent: true, opacity: 0.5, linewidth: 1 });
                const wireframe = new THREE.LineSegments(edgesGeo, lineMat);
                
                child.material = new THREE.MeshBasicMaterial( {transparent: true, opacity: 0} );
                child.add(wireframe);
            }
        });

        scene.add(object);
    }, undefined, (error) => console.error(`OBJ error: ${error}`));

    function animate(time) {
        renderer.render(scene, camera);

        if (sailboat !== undefined) {
            //sailboat.rotation.x = time / 1000;
            sailboat.rotation.y = time / 5000;
        }
    }

    renderer.setAnimationLoop(animate);
}
