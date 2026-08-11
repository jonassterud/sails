import * as THREE from "three"
import { OBJLoader } from 'three/addons/loaders/OBJLoader.js';
import { MTLLoader } from 'three/addons/loaders/MTLLoader.js';
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';

/* Scene and renderer */

const scene = new THREE.Scene();
const renderer = new THREE.WebGLRenderer();

/* Lights */

const ambientLight = new THREE.AmbientLight(0xffffff, 1.5);
const dirLight = new THREE.DirectionalLight(0xffffff, 2.5);

/* Timer */

const timer = new THREE.Timer();

/* Camera and controls */

const camera = new THREE.PerspectiveCamera(90, window.innerWidth / window.innerHeight, 0.1, 10000);
const controls = new OrbitControls(camera, renderer.domElement);

/* Objects and loaders */

const objects = new Map();
const objLoader = new OBJLoader();
const createMarker = (x, y, z, radius) => {
    const geometry = new THREE.SphereGeometry(radius, 32, 16);
    const material = new THREE.MeshBasicMaterial({ color: 0xffff00, transparent: true, opacity: 0.8, depthTest: false, depthWrite: false });
    const object = new THREE.Mesh(geometry, material);

    object.position.set(x, y, z);
    object.userData = { hover: false, active: false };

    return object;
};

/* Check marker hits */

const raycaster = new THREE.Raycaster();
const mouse = new THREE.Vector2();

function getHit(event, objects) {
    let hit = undefined;

    // Create ray

    mouse.x = (event.clientX / window.innerWidth) * 2 - 1;
    mouse.y = -(event.clientY / window.innerHeight) * 2 + 1;
    raycaster.setFromCamera(mouse, camera);

    // Check collisions
    const intersects = raycaster.intersectObjects(scene.children, true);

    for (let i = 0; i < intersects.length; i++) {
        if (objects.includes(intersects[i].object)) {
            hit = intersects[i];
            break;
        }
    }

    return hit;
}

function onMouseMove(event) {
    const markers = [...objects.entries().filter((kv) => kv[0].startsWith("marker")).map((kv) => kv[1])];
    const hit = getHit(event, markers);
    
    markers.forEach((marker) => {
        marker.userData.hover = false;
        marker.material.opacity = 0.8;
        document.body.style.cursor = "default";
    });
    
    if (hit !== undefined) {
        hit.object.userData.hover = true;
        hit.object.material.opacity = 1.0;
        document.body.style.cursor = "pointer";
    }
}

function onMouseDown(event) {
    const markers = [...objects.entries().filter((kv) => kv[0].startsWith("marker"))]
    const hit = getHit(event, markers);

    if (hit !== undefined) {
        hit.object.userData.active = !hit.object.userData.active;
    }
}

/* Initialization */

async function initialize() {
    // Configure timer

    timer.connect(document);

    // Setup scene

    camera.position.set(0, 0, -10);
    scene.background = new THREE.Color("black");
    renderer.setSize(window.innerWidth, window.innerHeight);
    document.body.appendChild(renderer.domElement);
    scene.add(controls.object);

    // Configure lights

    dirLight.position.set(5, 10, 7.5);
    scene.add(ambientLight);
    scene.add(dirLight);

    // Load sailboat

    const sailboat = await objLoader.loadAsync("assets/sailboat.obj");
    objects.set("sailboat", sailboat);
    scene.add(sailboat);

    // Create wireframe
    
    sailboat.traverse((child) => {
        if (child.isMesh) {
            const edgesGeo = new THREE.EdgesGeometry(child.geometry);
            const lineMat = new THREE.LineBasicMaterial({ color: 0xffffff, transparent: true, opacity: 0.5, linewidth: 1 });
            const wireframe = new THREE.LineSegments(edgesGeo, lineMat);

            child.material = new THREE.MeshBasicMaterial( {transparent: true, opacity: 0} );
            child.add(wireframe);
        }
    });

    // Create markers

    const marker_1 = createMarker(0, -5, 0, 0.2);
    objects.get("sailboat").add(marker_1);
    objects.set("marker_1", marker_1);

    const marker_2 = createMarker(-3, -5, 0, 0.2);
    objects.get("sailboat").add(marker_2);
    objects.set("marker_2", marker_2);

    const marker_3 = createMarker(3, -5, 0, 0.2);
    objects.get("sailboat").add(marker_3);
    objects.set("marker_3", marker_3);

    // Start loop

    renderer.setAnimationLoop(loop);
}

/* Render loop */

function loop() {
    // Update time

    timer.update();
    const delta = timer.getDelta();

    // Update controls

    controls.update();

    // Render scene

    renderer.render(scene, camera);
}

/* Event listeners */

window.addEventListener("load", initialize);
window.addEventListener("mousemove", onMouseMove);
window.addEventListener("click", onMouseDown);
